<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Update ApprovalRule by key</name>
   <tag></tag>
   <elementGuidId>538a62f6-2809-42bc-98ec-204186bade65</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;version\&quot; : 1,\n  \&quot;actions\&quot; : [ {\n    \&quot;action\&quot; : \&quot;setStatus\&quot;,\n    \&quot;status\&quot; : \&quot;Active\&quot;\n  } ]\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer {{ctp_access_token}}</value>
      <webElementGuid>5b5fcf90-304c-4e3b-bd75-806a288a66f4</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>73f9f9bc-cd6a-44b3-8a1c-366faa89f788</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.9</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${host}/${project-key}/as-associate/${associate-id}/in-business-unit/key=${business-unit-key}/approval-rules/key=${approval-rule-key}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.host</defaultValue>
      <description></description>
      <id>4261ae51-d0a6-4607-bf7e-f8c6a828bf39</id>
      <masked>false</masked>
      <name>host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.project-key</defaultValue>
      <description></description>
      <id>a50df036-39ec-466f-ac0b-733fa9b8b5ac</id>
      <masked>false</masked>
      <name>project-key</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.associate-id</defaultValue>
      <description></description>
      <id>dfa87c82-d6b2-4305-b439-03edbc2d56ab</id>
      <masked>false</masked>
      <name>associate-id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.business-unit-key</defaultValue>
      <description></description>
      <id>6362f527-e36a-4a49-bdbf-acfdf4658a3f</id>
      <masked>false</masked>
      <name>business-unit-key</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.approval-rule-key</defaultValue>
      <description></description>
      <id>7f28f233-f9c5-4d17-9758-d9d114f7bee0</id>
      <masked>false</masked>
      <name>approval-rule-key</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
